//! Retromancer — `{2}{R}{R}` 3/3 red Lizard Shaman. "Whenever this
//! creature becomes the target of a spell or ability, this creature
//! deals 3 damage to that spell or ability's controller."
//!
//! GAP: the catalog only documents `trig.triggering_caster()` as
//! pairing with `SpellCast`. Under `SelfBecomesTarget` it may return
//! `None`, in which case the effect no-ops. Emitted as best-effort —
//! the engine team will plumb the spell/ability controller through
//! the `SelfBecomesTarget` event later.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Retromancer");
    let lizard = reg.interner_mut().intern("Lizard");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: deal_three_to_caster,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Trigger resolution: Retromancer deals 3 damage to the controller
/// of the spell or ability that targeted it.
fn deal_three_to_caster(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: `triggering_caster()` is documented as pairing with
    // `SpellCast`; under `SelfBecomesTarget` the controller of the
    // triggering spell/ability may not be exposed yet. If it is
    // `None`, we emit no effects rather than damage a wrong player.
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(caster),
        amount: 3,
        source: trig.source,
    }]
}
