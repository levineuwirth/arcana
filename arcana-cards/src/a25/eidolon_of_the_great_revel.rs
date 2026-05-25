//! Eidolon of the Great Revel — `{R}{R}` 2/2 red Enchantment Creature — Spirit.
//! "Whenever a player casts a spell with mana value 3 or less, this
//! creature deals 2 damage to that player."

use arcana_core::effects::Effect;
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eidolon of the Great Revel");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: None,
                        ..Default::default()
                    }.with_max_cmc(3)),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_spell_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_spell_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The caster is the player who cast the spell. In this shape the
    // trigger fires for any player; trig.controller is this card's
    // controller, not the caster. GAP: no field on PendingTrigger
    // directly exposes the casting player for SpellCast triggers.
    // Using trig.controller as approximation.
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: 2,
        source: trig.source,
    }]
}
