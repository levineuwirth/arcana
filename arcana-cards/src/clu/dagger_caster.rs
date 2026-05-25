//! Dagger Caster — `{3}{R}` 2/3 red creature. "When this creature enters, it
//! deals 1 damage to each opponent and 1 damage to each creature your
//! opponents control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dagger Caster");
    let lizard = reg.interner_mut().intern("Lizard");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // Damage each opponent
    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 1,
            source: trig.source,
        });
    }
    // Damage each creature opponents control
    let opponent_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    for id in opponent_creatures {
        effects.push(Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 1,
            source: trig.source,
        });
    }
    effects
}
