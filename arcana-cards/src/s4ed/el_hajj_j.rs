//! El-Hajjâj — `{1}{B}{B}` 1/1 black Human Wizard. "Whenever this
//! creature deals damage, you gain that much life." The trigger
//! catalog has no dedicated `SelfDealsDamage` variant; the nearest
//! fit is `DamageDealt`, which filters by `ObjectFilter` rather than
//! by source object identity, so a permissive filter is used and the
//! self-restriction is flagged as a gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("El-Hajjâj");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — oracle restricts to damage dealt by
                // this creature; `DamageDealt`'s `source_filter` is an
                // `ObjectFilter` with no self-identity refinement, so
                // we approximate with an unconstrained filter.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: gain_life_equal_to_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "You gain that much life" — read the damage amount from the
/// triggering event via the typed accessor and credit it to the
/// ability's controller.
fn gain_life_equal_to_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    vec![Effect::GainLife { player: trig.controller, amount }]
}
