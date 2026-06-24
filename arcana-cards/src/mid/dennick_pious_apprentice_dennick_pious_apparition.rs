//! Dennick, Pious Apprentice // Dennick, Pious Apparition
//!
//! FRONT: {W}{U} — Legendary Creature — Human Soldier (2/3)
//! Lifelink.
//! Cards in graveyards can't be the targets of spells or abilities. (GAP: static graveyard
//! protection effect not modeled.)
//! Disturb {2}{W}{U} — You may cast this from your graveyard transformed for its disturb cost.
//! (GAP: Disturb keyword not in the engine keyword surface.)
//!
//! BACK: Dennick, Pious Apparition — Legendary Creature — Spirit Soldier (3/4)
//! Flying.
//! Whenever one or more creature cards are put into graveyards from anywhere, investigate.
//! This ability triggers only once each turn. (Wired: ZoneChange creature→graveyard,
//! OncePerTurn, creates a Clue.)
//! If Dennick would be put into a graveyard from anywhere, exile it instead.
//! (Wired: ExileInsteadOfDying replacement.)

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dennick, Pious Apprentice");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");

    let mut front_subtypes = SubtypeSet::new();
    front_subtypes.insert(human_sub);
    front_subtypes.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dennick, Pious Apparition");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let soldier_sub2 = reg.interner_mut().intern("Soldier");

    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(spirit_sub);
    back_subtypes.insert(soldier_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back face: "Whenever one or more creature cards are put into graveyards
            // from anywhere, investigate. This ability triggers only once each turn."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            // Back face: "If Dennick would be put into a graveyard from anywhere,
            // exile it instead." Installed on transform-to-back and on entering as
            // the back face (Disturb).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_exile_replacement,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_exile_replacement,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // All three are back-face abilities — gate to face 1.
            .with_trigger_face_gate(1, 1)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
    )
}

/// "…investigate." — create one Clue token for the controller.
fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}

/// "If Dennick would be put into a graveyard from anywhere, exile it instead."
/// (battlefield→graveyard case observed.)
fn install_exile_replacement(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
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
    }]
}
