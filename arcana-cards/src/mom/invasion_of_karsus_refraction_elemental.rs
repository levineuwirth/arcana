//! Invasion of Karsus // Refraction Elemental — {2}{R}{R} Battle — Siege (red).
//!
//! Front (Siege): When this Siege enters, it deals 3 damage to each creature
//! and each planeswalker.
//! Back (Refraction Elemental, Creature — Elemental): Ward—Pay 2 life;
//! Whenever you cast a spell, this creature deals 2 damage to each opponent.
//!
//! GAP: Ward—Pay 2 life is a non-mana ward cost; not expressible as
//! KeywordAbility::Ward (which takes a ManaCost) — keywords left empty on the back.
//!
//! Defeat→back-face is auto-wired by the engine SBA. Front ETB (damage sweep) is
//! face-gated to the battle face (0); the back-face "whenever you cast a spell,
//! deal 2 to each opponent" is face-gated to the creature face (1).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Karsus");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Refraction Elemental");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 3,
            })
            .with_transform_back(back)
            // Front (Siege) ETB: deal 3 damage to each creature and each planeswalker.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (Refraction Elemental): whenever you cast a spell, deals 2 damage
            // to each opponent. Face-gated to the back face (1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_cast_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn etb_damage(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    let pw_filter = ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into());
    let walkers = script::ids_matching(state, &pw_filter, trig.controller);
    let mut effs = Vec::new();
    for id in creatures.into_iter().chain(walkers.into_iter()) {
        effs.push(Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 3,
            source: trig.source,
        });
    }
    effs
}

fn back_cast_damage(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 2,
            source: trig.source,
        })
        .collect()
}
