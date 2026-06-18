//! Arvad of the Weatherlight — `{2}{W}{B}` 3/4 Legendary Human Knight.
//! "Rulebreaker — (Commander deck-building rule). Whenever Arvad deals
//! combat damage to an opponent, you may put a legendary permanent card
//! from your hand onto the battlefield."
//!
//! Rulebreaker is a deck-construction static with no in-game effect —
//! GAP'd (no engine surface). The combat-damage trigger puts a legendary
//! permanent from hand onto the battlefield (PutFromHandOntoBattlefield
//! with a legendary-supertype filter).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arvad of the Weatherlight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    // GAP: Rulebreaker (Commander color-identity deck-building rule) — no
    // in-game effect; not modeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: put_legendary,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn put_legendary(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
        tapped: false,
    }]
}
