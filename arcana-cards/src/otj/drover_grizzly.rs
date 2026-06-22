//! Drover Grizzly — `{2}{G}` 4/2 green Creature — Bear Mount.
//!
//! Whenever this creature attacks while saddled, creatures you control
//!   gain trample until end of turn.
//! Saddle 1
//!
//! Decomposition: the attack trigger → one `TriggeredAbilityDef`
//! (`SelfAttacks`) that grants trample to every creature you control via
//! `Effect::ForEach` over `GrantKeyword`. Saddle 1 is NOT in the usable
//! keyword surface (the saddle activation — tap creatures totaling power
//! 1+, becomes saddled until end of turn — is unmodeled), so
//! `keywords: vec![]`. The "while saddled" gate on the trigger is also
//! unmodeled (no saddled-state condition helper), so it fires
//! unconditionally — a documented fidelity gap matching the Gloryheath
//! Lynx precedent.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drover Grizzly");
    let bear = reg.interner_mut().intern("Bear");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(mount);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword "Saddle 1" — the saddle activation (tap creatures
        // totaling power 1+, becomes saddled until EOT, sorcery-speed) is
        // not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_grant_trample,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_grant_trample(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        }),
    }]
}
