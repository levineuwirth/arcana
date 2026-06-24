//! Sumala Rumblers — `{2}{G/W}{G/W}` */4 Wurm.
//! "Sumala Rumblers's power is equal to the number of creatures you
//!   control." — a characteristic-defining ability; power is the `*`
//!   resolved at Layer 7a via an ETB self-CDA (scalar compute), toughness
//!   the printed 4 (returned unchanged so the 7a SET preserves it).
//! Myriad — GAP (not a modeled KeywordAbility; the attack-time token-copy
//!   mechanic is unexpressible).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sumala Rumblers");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    // Colors: G, W (the {G/W} hybrid pips make it green-white).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // "power is equal to the number of creatures you control" is installed
        // at Layer 7a via the ETB self-CDA below; toughness stays the printed 4.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Myriad — not a modeled KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == who && o.characteristics.types.is_creature())
        .count() as i32;
    // Only power is `*`; the SET overwrites both, so return the printed 4.
    (n, 4)
}
