// Generated by gir (https://github.com/gtk-rs/gir @ b75e059)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7d95377)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 85bd06b)
// DO NOT EDIT

use gstreamer_webrtc_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-webrtc-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstWebRTCBundlePolicy",
        Layout {
            size: size_of::<GstWebRTCBundlePolicy>(),
            alignment: align_of::<GstWebRTCBundlePolicy>(),
        },
    ),
    (
        "GstWebRTCDTLSSetup",
        Layout {
            size: size_of::<GstWebRTCDTLSSetup>(),
            alignment: align_of::<GstWebRTCDTLSSetup>(),
        },
    ),
    (
        "GstWebRTCDTLSTransport",
        Layout {
            size: size_of::<GstWebRTCDTLSTransport>(),
            alignment: align_of::<GstWebRTCDTLSTransport>(),
        },
    ),
    (
        "GstWebRTCDTLSTransportClass",
        Layout {
            size: size_of::<GstWebRTCDTLSTransportClass>(),
            alignment: align_of::<GstWebRTCDTLSTransportClass>(),
        },
    ),
    (
        "GstWebRTCDTLSTransportState",
        Layout {
            size: size_of::<GstWebRTCDTLSTransportState>(),
            alignment: align_of::<GstWebRTCDTLSTransportState>(),
        },
    ),
    (
        "GstWebRTCDataChannel",
        Layout {
            size: size_of::<GstWebRTCDataChannel>(),
            alignment: align_of::<GstWebRTCDataChannel>(),
        },
    ),
    (
        "GstWebRTCDataChannelClass",
        Layout {
            size: size_of::<GstWebRTCDataChannelClass>(),
            alignment: align_of::<GstWebRTCDataChannelClass>(),
        },
    ),
    (
        "GstWebRTCDataChannelState",
        Layout {
            size: size_of::<GstWebRTCDataChannelState>(),
            alignment: align_of::<GstWebRTCDataChannelState>(),
        },
    ),
    (
        "GstWebRTCFECType",
        Layout {
            size: size_of::<GstWebRTCFECType>(),
            alignment: align_of::<GstWebRTCFECType>(),
        },
    ),
    (
        "GstWebRTCICEComponent",
        Layout {
            size: size_of::<GstWebRTCICEComponent>(),
            alignment: align_of::<GstWebRTCICEComponent>(),
        },
    ),
    (
        "GstWebRTCICEConnectionState",
        Layout {
            size: size_of::<GstWebRTCICEConnectionState>(),
            alignment: align_of::<GstWebRTCICEConnectionState>(),
        },
    ),
    (
        "GstWebRTCICEGatheringState",
        Layout {
            size: size_of::<GstWebRTCICEGatheringState>(),
            alignment: align_of::<GstWebRTCICEGatheringState>(),
        },
    ),
    (
        "GstWebRTCICERole",
        Layout {
            size: size_of::<GstWebRTCICERole>(),
            alignment: align_of::<GstWebRTCICERole>(),
        },
    ),
    (
        "GstWebRTCICETransport",
        Layout {
            size: size_of::<GstWebRTCICETransport>(),
            alignment: align_of::<GstWebRTCICETransport>(),
        },
    ),
    (
        "GstWebRTCICETransportClass",
        Layout {
            size: size_of::<GstWebRTCICETransportClass>(),
            alignment: align_of::<GstWebRTCICETransportClass>(),
        },
    ),
    (
        "GstWebRTCICETransportPolicy",
        Layout {
            size: size_of::<GstWebRTCICETransportPolicy>(),
            alignment: align_of::<GstWebRTCICETransportPolicy>(),
        },
    ),
    (
        "GstWebRTCKind",
        Layout {
            size: size_of::<GstWebRTCKind>(),
            alignment: align_of::<GstWebRTCKind>(),
        },
    ),
    (
        "GstWebRTCPeerConnectionState",
        Layout {
            size: size_of::<GstWebRTCPeerConnectionState>(),
            alignment: align_of::<GstWebRTCPeerConnectionState>(),
        },
    ),
    (
        "GstWebRTCPriorityType",
        Layout {
            size: size_of::<GstWebRTCPriorityType>(),
            alignment: align_of::<GstWebRTCPriorityType>(),
        },
    ),
    (
        "GstWebRTCRTPReceiver",
        Layout {
            size: size_of::<GstWebRTCRTPReceiver>(),
            alignment: align_of::<GstWebRTCRTPReceiver>(),
        },
    ),
    (
        "GstWebRTCRTPReceiverClass",
        Layout {
            size: size_of::<GstWebRTCRTPReceiverClass>(),
            alignment: align_of::<GstWebRTCRTPReceiverClass>(),
        },
    ),
    (
        "GstWebRTCRTPSender",
        Layout {
            size: size_of::<GstWebRTCRTPSender>(),
            alignment: align_of::<GstWebRTCRTPSender>(),
        },
    ),
    (
        "GstWebRTCRTPSenderClass",
        Layout {
            size: size_of::<GstWebRTCRTPSenderClass>(),
            alignment: align_of::<GstWebRTCRTPSenderClass>(),
        },
    ),
    (
        "GstWebRTCRTPTransceiver",
        Layout {
            size: size_of::<GstWebRTCRTPTransceiver>(),
            alignment: align_of::<GstWebRTCRTPTransceiver>(),
        },
    ),
    (
        "GstWebRTCRTPTransceiverClass",
        Layout {
            size: size_of::<GstWebRTCRTPTransceiverClass>(),
            alignment: align_of::<GstWebRTCRTPTransceiverClass>(),
        },
    ),
    (
        "GstWebRTCRTPTransceiverDirection",
        Layout {
            size: size_of::<GstWebRTCRTPTransceiverDirection>(),
            alignment: align_of::<GstWebRTCRTPTransceiverDirection>(),
        },
    ),
    (
        "GstWebRTCSCTPTransportState",
        Layout {
            size: size_of::<GstWebRTCSCTPTransportState>(),
            alignment: align_of::<GstWebRTCSCTPTransportState>(),
        },
    ),
    (
        "GstWebRTCSDPType",
        Layout {
            size: size_of::<GstWebRTCSDPType>(),
            alignment: align_of::<GstWebRTCSDPType>(),
        },
    ),
    (
        "GstWebRTCSessionDescription",
        Layout {
            size: size_of::<GstWebRTCSessionDescription>(),
            alignment: align_of::<GstWebRTCSessionDescription>(),
        },
    ),
    (
        "GstWebRTCSignalingState",
        Layout {
            size: size_of::<GstWebRTCSignalingState>(),
            alignment: align_of::<GstWebRTCSignalingState>(),
        },
    ),
    (
        "GstWebRTCStatsType",
        Layout {
            size: size_of::<GstWebRTCStatsType>(),
            alignment: align_of::<GstWebRTCStatsType>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GST_WEBRTC_BUNDLE_POLICY_BALANCED", "1"),
    ("(gint) GST_WEBRTC_BUNDLE_POLICY_MAX_BUNDLE", "3"),
    ("(gint) GST_WEBRTC_BUNDLE_POLICY_MAX_COMPAT", "2"),
    ("(gint) GST_WEBRTC_BUNDLE_POLICY_NONE", "0"),
    ("(gint) GST_WEBRTC_DATA_CHANNEL_STATE_CLOSED", "4"),
    ("(gint) GST_WEBRTC_DATA_CHANNEL_STATE_CLOSING", "3"),
    ("(gint) GST_WEBRTC_DATA_CHANNEL_STATE_CONNECTING", "1"),
    ("(gint) GST_WEBRTC_DATA_CHANNEL_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_DATA_CHANNEL_STATE_OPEN", "2"),
    ("(gint) GST_WEBRTC_DTLS_SETUP_ACTIVE", "2"),
    ("(gint) GST_WEBRTC_DTLS_SETUP_ACTPASS", "1"),
    ("(gint) GST_WEBRTC_DTLS_SETUP_NONE", "0"),
    ("(gint) GST_WEBRTC_DTLS_SETUP_PASSIVE", "3"),
    ("(gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED", "1"),
    ("(gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED", "4"),
    ("(gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING", "3"),
    ("(gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED", "2"),
    ("(gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_FEC_TYPE_NONE", "0"),
    ("(gint) GST_WEBRTC_FEC_TYPE_ULP_RED", "1"),
    ("(gint) GST_WEBRTC_ICE_COMPONENT_RTCP", "1"),
    ("(gint) GST_WEBRTC_ICE_COMPONENT_RTP", "0"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING", "1"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED", "6"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED", "3"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED", "2"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED", "5"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_FAILED", "4"),
    ("(gint) GST_WEBRTC_ICE_CONNECTION_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE", "2"),
    ("(gint) GST_WEBRTC_ICE_GATHERING_STATE_GATHERING", "1"),
    ("(gint) GST_WEBRTC_ICE_GATHERING_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_ICE_ROLE_CONTROLLED", "0"),
    ("(gint) GST_WEBRTC_ICE_ROLE_CONTROLLING", "1"),
    ("(gint) GST_WEBRTC_ICE_TRANSPORT_POLICY_ALL", "0"),
    ("(gint) GST_WEBRTC_ICE_TRANSPORT_POLICY_RELAY", "1"),
    ("(gint) GST_WEBRTC_KIND_AUDIO", "1"),
    ("(gint) GST_WEBRTC_KIND_UNKNOWN", "0"),
    ("(gint) GST_WEBRTC_KIND_VIDEO", "2"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED", "5"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED", "2"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING", "1"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED", "3"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_FAILED", "4"),
    ("(gint) GST_WEBRTC_PEER_CONNECTION_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_PRIORITY_TYPE_HIGH", "4"),
    ("(gint) GST_WEBRTC_PRIORITY_TYPE_LOW", "2"),
    ("(gint) GST_WEBRTC_PRIORITY_TYPE_MEDIUM", "3"),
    ("(gint) GST_WEBRTC_PRIORITY_TYPE_VERY_LOW", "1"),
    ("(gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_INACTIVE", "1"),
    ("(gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_NONE", "0"),
    ("(gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_RECVONLY", "3"),
    ("(gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDONLY", "2"),
    ("(gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDRECV", "4"),
    ("(gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CLOSED", "3"),
    ("(gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTED", "2"),
    ("(gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTING", "1"),
    ("(gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_NEW", "0"),
    ("(gint) GST_WEBRTC_SDP_TYPE_ANSWER", "3"),
    ("(gint) GST_WEBRTC_SDP_TYPE_OFFER", "1"),
    ("(gint) GST_WEBRTC_SDP_TYPE_PRANSWER", "2"),
    ("(gint) GST_WEBRTC_SDP_TYPE_ROLLBACK", "4"),
    ("(gint) GST_WEBRTC_SIGNALING_STATE_CLOSED", "1"),
    ("(gint) GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER", "2"),
    ("(gint) GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER", "4"),
    ("(gint) GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER", "3"),
    (
        "(gint) GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER",
        "5",
    ),
    ("(gint) GST_WEBRTC_SIGNALING_STATE_STABLE", "0"),
    ("(gint) GST_WEBRTC_STATS_CANDIDATE_PAIR", "11"),
    ("(gint) GST_WEBRTC_STATS_CERTIFICATE", "14"),
    ("(gint) GST_WEBRTC_STATS_CODEC", "1"),
    ("(gint) GST_WEBRTC_STATS_CSRC", "6"),
    ("(gint) GST_WEBRTC_STATS_DATA_CHANNEL", "8"),
    ("(gint) GST_WEBRTC_STATS_INBOUND_RTP", "2"),
    ("(gint) GST_WEBRTC_STATS_LOCAL_CANDIDATE", "12"),
    ("(gint) GST_WEBRTC_STATS_OUTBOUND_RTP", "3"),
    ("(gint) GST_WEBRTC_STATS_PEER_CONNECTION", "7"),
    ("(gint) GST_WEBRTC_STATS_REMOTE_CANDIDATE", "13"),
    ("(gint) GST_WEBRTC_STATS_REMOTE_INBOUND_RTP", "4"),
    ("(gint) GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP", "5"),
    ("(gint) GST_WEBRTC_STATS_STREAM", "9"),
    ("(gint) GST_WEBRTC_STATS_TRANSPORT", "10"),
];
