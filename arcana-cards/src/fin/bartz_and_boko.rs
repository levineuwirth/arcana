//! Bartz and Boko — `{3}{G}{G}` Legendary 4/3 green Human Bird.
//!
//! Rules text:
//! * Affinity for Birds. (Affinity is not in the usable KeywordAbility
//!   surface — emitted as `keywords: vec![]` and GAP'd.)
//! * When Bartz and Boko enters, each other Bird you control deals damage
//!   equal to its power to target creature an opponent controls.
//!
//! The ETB is a targeted trigger: it picks one target creature an opponent
//! controls, then every OTHER Bird you control deals damage equal to that
//! Bird's power to the chosen target. Resolved by enumerating your Birds
//! (excluding the source) and emitting one `DealDamage` per Bird, each with
//! amount = that Bird's power and source = that Bird.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bartz and Boko");
    let human = reg.interner_mut().intern("Human");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Affinity is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_birds_deal_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_birds_deal_damage(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(tid) = target else { return Vec::new(); };

    // Each OTHER Bird you control deals damage equal to its power.
    let birds = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Bird").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects = Vec::new();
    for bird in birds {
        if bird == trig.source {
            continue;
        }
        let amount = script::power_of(state, bird).max(0) as u32;
        effects.push(Effect::DealDamage {
            source: bird,
            target: DamageTarget::Object(*tid),
            amount,
        });
    }
    effects
}
