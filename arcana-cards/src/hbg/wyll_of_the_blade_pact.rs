//! Wyll of the Blade Pact — `{3}{R}{R}{R}` Legendary 5/5 red Human
//! Warlock. "When this creature specializes, you may sacrifice another
//! creature or an artifact. If you do, untap Wyll of the Blade Pact.
//! After this main phase, there is an additional combat phase followed
//! by an additional main phase."
//!
//! Partial: the specialize EVENT fires the trigger (the colored-back
//! face swap is unmodeled). The "you may sacrifice another creature or
//! an artifact" gate is not expressible — `OptionalPayment` only
//! supports Mana / Life costs, not a sacrifice cost — so the untap is
//! emitted unconditionally as a best effort. The "additional combat
//! phase followed by an additional main phase" extra-phase insertion
//! has no catalog primitive and is GAP-ed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wyll of the Blade Pact");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: on_specialize,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_specialize(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature or an artifact" gate not
    // expressible (OptionalPayment supports only Mana/Life costs, not a
    // sacrifice cost) — untap emitted unconditionally as best effort.
    // GAP: "an additional combat phase followed by an additional main
    // phase" has no extra-phase primitive in the catalog.
    vec![Effect::Untap { target: trig.source }]
}
