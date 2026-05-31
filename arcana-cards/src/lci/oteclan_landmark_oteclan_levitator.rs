//! Oteclan Landmark // Oteclan Levitator — transforming DFC (artifact // artifact creature).
//!
//! Front (Oteclan Landmark): Artifact {W}.
//!   When this artifact enters, scry 2.
//!   Craft with artifact {2}{W} — transform mechanic; the Craft activated ability is
//!   engine debt (GAP).
//! Back (Oteclan Levitator): Artifact Creature — Golem 3/3 with Flying.
//!   Whenever this creature attacks, target attacking creature without flying gains
//!   flying until end of turn. (back-only trigger, face-gated to face 1)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oteclan Landmark");
    let back_name = reg.interner_mut().intern("Oteclan Levitator");
    let golem = reg.interner_mut().intern("Golem");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(golem);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: When this artifact enters, scry 2.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: Whenever this creature attacks, target attacking creature without
            // flying gains flying until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(2, 1)
            // GAP: Craft with artifact {2}{W} — the Craft transform-from-exile activated
            // ability (exile this + another artifact, return transformed) is not modeled.
            // GAP: "target ATTACKING creature WITHOUT flying" restriction not expressible in
            // the target filter; targets any creature instead.
    )
}

fn etb_scry(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry {
        player: trig.controller,
        count: 2,
    }]
}

fn grant_flying(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}
