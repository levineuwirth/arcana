//! Chaplain of Alms // Chapel Shieldgeist — `{W}` Creature — Human Cleric 1/1 (front) /
//! Creature — Spirit Cleric (back). Transform (Disturb).
//!
//! Front face:
//!   First strike
//!   Ward {1}
//!   Disturb {3}{W} (cast from graveyard transformed; not modeled — engine debt)
//!
//! Back face (Chapel Shieldgeist):
//!   Flying, first strike
//!   Each creature you control has ward {1}. (filtered_keyword static, installed on
//!     transform-to-back and on entering as the back face via Disturb.)
//!   If Chapel Shieldgeist would be put into a graveyard from anywhere, exile it instead.
//!     (ExileInsteadOfDying replacement, installed alongside the static.)
//!
//! GAP: Disturb (cast from graveyard transformed) — engine debt; not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaplain of Alms");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid ward cost")),
        ],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Chapel Shieldgeist");
    let back_spirit_sub = reg.interner_mut().intern("Spirit");
    let back_cleric_sub = reg.interner_mut().intern("Cleric");

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_spirit_sub);
    back_subtypes.0.insert(back_cleric_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: Disturb — cast from graveyard transformed — engine debt; not modeled.
            //
            // Back-face statics (Chapel Shieldgeist): installed when transforming to
            // the back face AND when entering directly as the back face via Disturb.
            // Both triggers gated to face 1 so they never fire on the front creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_back_statics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_back_statics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1)
            .with_trigger_face_gate(2, 1),
    )
}

/// Install the back-face statics: "each creature you control has ward {1}" plus the
/// "if Chapel Shieldgeist would be put into a graveyard from anywhere, exile it
/// instead" replacement. Both auto-expire when the source leaves the battlefield.
fn install_back_statics(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ward = KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid ward cost"));
    let creatures_you_control =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                creatures_you_control,
                ward,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        // "from anywhere" is templated; the battlefield→graveyard case (a creature
        // dying) is the dominant one and the only one this WhileSourceOnBattlefield
        // replacement can observe.
        Effect::InstallReplacementEffect {
            effect: Box::new(ReplacementEffect {
                source: trig.source,
                id: 0,
                condition: ReplacementCondition::WouldDieSpecific {
                    object_id: trig.source,
                },
                kind: ReplacementKind::ExileInsteadOfDying,
                is_self_replacement: true,
                duration: ReplacementDuration::WhileSourceOnBattlefield,
                state_gate: None,
            }),
        },
    ]
}
