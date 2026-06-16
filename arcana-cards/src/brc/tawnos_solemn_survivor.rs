//! Tawnos, Solemn Survivor — `{1}{U}` Legendary 1/3 Human Artificer.
//!
//! Scryfall lists keyword "Mill", but Mill is not a usable
//! `KeywordAbility` variant; the milling is part of an activated
//! ability's effect, not a keyword. keywords: vec![].
//!
//! Ability 1: "{2}, {T}: Create a token that's a copy of up to one
//!   target artifact token you control. Mill two cards." — expressed
//!   as CopyPermanent (if a target was chosen) then Mill 2.
//! Ability 2: "{1}{W}{U}{B}, {T}, Sacrifice two artifact tokens,
//!   Exile an artifact or creature card from your graveyard: Create a
//!   token that's a copy of the exiled card... Activate only as a
//!   sorcery." — best-expressible cost (mana + tap + sacrifice two
//!   artifact tokens, sorcery speed). The graveyard-exile cost and
//!   the copy-the-exiled-card effect are not expressible; effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tawnos, Solemn Survivor");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Create a token that's a copy of up to one target artifact token you control. Mill two cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .tokens_only()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_token_and_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}{U}{B}, {T}, Sacrifice two artifact tokens, Exile an artifact or creature card from your graveyard: Create a token that's a copy of the exiled card, except it's an artifact in addition to its other types. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}{U}{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .tokens_only()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_exiled_card,
            }),
    )
}

/// {2},{T}: copy up to one target artifact token, then mill two.
fn copy_token_and_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::CopyPermanent { target: *id });
    }
    effects.push(Effect::Mill {
        player: ctx.controller,
        count: 2,
    });
    effects
}

/// Ability 2 effect.
fn copy_exiled_card(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exile an artifact or creature card from your graveyard" is
    // not an expressible ActivationCost field, and "create a token
    // that's a copy of the exiled card (as an artifact)" requires a
    // reference to the chosen exiled card that the engine cannot
    // capture here. Cost models mana + tap + sacrifice two artifact
    // tokens; the copy-the-exiled-card effect is GAP'd.
    Vec::new()
}
