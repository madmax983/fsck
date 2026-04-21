#![cfg(feature = "nova")]

use fsck::entity::Entity;
use fsck::experimental::ModemDialer;

#[test]
fn test_modem_dialer_escalation() {
    let mut entity = Entity::new();
    let seed = 42;

    // Surface layer
    entity.update_depth(0);
    let surface_output = ModemDialer::dial("555-1234", &entity, seed);
    assert!(surface_output.contains("DIALING 555-1234..."));
    assert!(surface_output.contains("NO CARRIER"));

    // Corruption layer
    entity.update_depth(10);
    let corruption_output = ModemDialer::dial("555-1234", &entity, seed);
    assert!(corruption_output.contains("RING..."));
    assert!(corruption_output.contains("STATIC"));

    // Presence layer
    entity.update_depth(20);
    let mut presence_found = false;
    for s in 0..50 {
        let presence_output = ModemDialer::dial("555-1234", &entity, s);
        if presence_output.contains("THEY CANNOT HEAR YOU") {
            presence_found = true;
            break;
        }
    }
    assert!(presence_found, "Should generate 'THEY CANNOT HEAR YOU' with at least one seed at Presence layer");

    // Infection layer
    entity.update_depth(30);
    let mut infection_found = false;
    for s in 0..50 {
        let infection_output = ModemDialer::dial("555-1234", &entity, s);
        if infection_output.contains("I AM THE ONLY ONE LISTENING") {
            infection_found = true;
            break;
        }
    }
    assert!(infection_found, "Should generate 'I AM THE ONLY ONE LISTENING' with at least one seed at Infection layer");
}

#[test]
fn test_modem_dialer_easter_eggs() {
    let entity = Entity::new();
    let seed = 42;

    let output = ModemDialer::dial("911", &entity, seed);
    assert!(output.contains("NO ONE IS COMING"));

    let output2 = ModemDialer::dial("867-5309", &entity, seed);
    assert!(output2.contains("SHE CANNOT HELP YOU"));
}
