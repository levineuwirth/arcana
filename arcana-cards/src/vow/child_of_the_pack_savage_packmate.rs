//! Child of the Pack // Savage Packmate — `{2}{R}{G}` Human Werewolf 2/5.
//! Front face: `{2}{R}{G}`: Create a 2/2 green Wolf creature token.
//! Daybound (GAP: day/night cycle not modeled).
//! Back face (Savage Packmate): Trample. Other creatures you control get +1/+0.
//! Nightbound (GAP: day/night cycle not modeled).
//!
//! GAP: Daybound/Nightbound keywords not in engine keyword set.
//! GAP: "{2}{R}{G}: Create a 2/2 green Wolf creature token." is an activated
//! ability on a permanent; CardDefinition has no activated-ability API for
//! transform cards in the shown reference patterns. Omitted.
//! Back-face "Other creatures you control get +1/+0" is modeled as a
//! filtered pump installed once from the ETB trigger with
//! `Duration::WhileSourceShowsFace(1)` — dimmed on the front face, live
//! on Savage Packmate. NOTE: no exclude-source filter builder, so the
//! source pumps itself +1/+0 too (accepted approximation of "other").
//! GAP: Day/night transform trigger conditions not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Child of the Pack");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Daybound keyword not in engine keyword set
        // GAP: activated ability {2}{R}{G}: Create Wolf token — no API for
        // permanent activated abilities in transform shape
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Savage Packmate");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Trample],
            // GAP: Nightbound keyword not in engine keyword set
            // "Other creatures you control get +1/+0" — installed from the
            // ETB trigger with Duration::WhileSourceShowsFace(1) below.
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Day/night transform trigger conditions not modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back-face static, installed once from the ETB trigger; the
            // face-gated duration keeps it dimmed while the front shows.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_back_face_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_back_face_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Savage Packmate: "Other creatures you control get +1/+0" — live only
    // while the back face shows. NOTE: no exclude-source filter builder, so
    // the source matches its own filter and pumps itself +1/+0 (accepted
    // approximation of "other").
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            1,
            0,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}
