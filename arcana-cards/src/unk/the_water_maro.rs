//! The Water Maro — `{1}{W}{U}{G}` */* Legendary Creature — Elemental.
//! "When you cast The Water Maro, create a token that's a copy of it, except
//!  the copy isn't legendary."
//! "The Water Maro's power and toughness are each equal to the number of
//!  permanents you control that are tokens and/or have the word 'Maro' in the
//!  name." (characteristic-defining ability — the */* base)
//!
//! The CDA P/T is `PtValue::Star`. The self-CDA engine resolves a single
//! count filter, but the count here is "permanents you control that are
//! tokens OR have the word 'Maro' in the name" — a disjunction of token
//! status and a NAME-SUBSTRING test, which no ObjectFilter predicate
//! expresses; left as */* (GAP). The cast trigger mints a token copy of this
//! spell via `Effect::CopyPermanent`; the "except the copy isn't legendary"
//! rider is not expressible (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Water Maro");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // GAP (CDA): "power and toughness are each equal to the number of permanents
    // you control that are tokens and/or have 'Maro' in the name" — a disjunction
    // of token status OR a name-substring test, which no ObjectFilter predicate
    // (nor the registry-free scalar fn) expresses; left as */* (PtValue::Star).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new()),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cast_copy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "create a token that's a copy of it" — copy this spell into a token.
    // GAP: "except the copy isn't legendary" — the legendary-stripping rider is
    // not expressible on CopyPermanent.
    vec![Effect::CopyPermanent {
        target: trig.source,
    }]
}
