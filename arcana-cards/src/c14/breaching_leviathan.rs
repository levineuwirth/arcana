//! Breaching Leviathan — `{7}{U}{U}` 9/9 blue creature (Leviathan).
//! "When this creature enters, if you cast it from your hand, tap all
//! nonblue creatures. Those creatures don't untap during their controllers'
//! next untap steps."
//!
//! GAP: "those creatures don't untap during their controllers' next untap
//! steps" — no engine Effect for suppressing untap. Emitting mass Tap
//! only; the untap-suppression rider is omitted.
//! GAP: "if you cast it from your hand" — intervening if for cast-from-hand
//! is not expressible; condition is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breaching Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening if — "if you cast it from your hand"
                // not expressible.
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .without_colors(ColorSet::blue());
    let ids = script::ids_matching(state, &filter, trig.controller);
    // GAP: "don't untap during their controllers' next untap steps" omitted.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Tap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
