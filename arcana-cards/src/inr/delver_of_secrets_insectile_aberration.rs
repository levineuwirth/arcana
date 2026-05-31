//! Delver of Secrets // Insectile Aberration (transforming DFC, layout "transform")
//!
//! Front face: Delver of Secrets — {U} Creature — Human Wizard, 1/1.
//!   At the beginning of your upkeep, look at the top card of your library. You may
//!   reveal that card. If an instant or sorcery card is revealed this way, transform
//!   this creature.
//! Back face: Insectile Aberration — Creature — Human Insect, 3/2, Flying.
//!
//! GAP: the front-face upkeep trigger ("look at the top card; you may reveal it; if an
//! instant or sorcery card is revealed this way, transform") is not expressible — there
//! is no primitive that peeks the top card and conditionally transforms based on the
//! revealed card's types. The transform structure (front->back) is authored as a
//! front-gated upkeep trigger that emits Effect::Transform, but the "if instant/sorcery
//! revealed" gate is the missing piece. (Authoring it unconditionally would transform
//! every upkeep, which is wrong, so the effect is GAP'd to Vec::new().)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Delver of Secrets");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let insect = reg.interner_mut().intern("Insect");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Insectile Aberration");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human);
    back_subtypes.0.insert(insect);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: at the beginning of your upkeep, look at the top card; you may
            // reveal it; if an instant or sorcery card is revealed this way, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: delver_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn delver_upkeep(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card of your library; you may reveal it; if an instant or
    // sorcery card is revealed this way, transform this creature." No primitive peeks the
    // top card and conditionally transforms on the revealed card's types. Emitting an
    // unconditional Effect::Transform would flip every upkeep regardless of the top card,
    // which is materially wrong, so the whole effect is GAP'd.
    Vec::new()
}
