//! Golden Guardian // Gold-Forge Garrison
//!
//! Front face: `{4}` Artifact Creature — Golem 4/4.
//! Defender.
//! "{2}: This creature fights another target creature you control. When this creature dies
//!  this turn, return it to the battlefield transformed under your control."
//!
//! Back face: Land — Gold-Forge Garrison.
//! "{T}: Add two mana of any one color."
//! "{4}, {T}: Create a 4/4 colorless Golem artifact creature token."
//!
//! # GAPs
//! - The fight activation's "when this creature dies this turn, return it to the battlefield
//!   transformed" is a delayed conditional trigger. The `DelayedAction` variant handles
//!   `ReturnFromExileToBattlefield` but not "return transformed". Emitting only `Fight`;
//!   the delayed transform-return is GAP'd.
//! - Back face "{T}: Add two mana of any one color" — modeled as five mana abilities, one
//!   per WUBRG color, each adding TWO mana of that color (the player picks the color by
//!   choosing which ability to activate; the shared {T} cost means only one fires). Gated
//!   to the back face (visible_face == 1).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golden Guardian");
    let golem_sub = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem_sub);
    // Pre-intern Golem for use in token creation at resolve time.
    let _golem_token_sub = reg.interner_mut().intern("Golem");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // Back face: Gold-Forge Garrison — Land
    let back_name = reg.interner_mut().intern("Gold-Forge Garrison");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "{2}: This creature fights another target creature you control."
            // GAP: The "when this creature dies this turn, return it transformed" rider
            // is not wired (no transform-on-return effect variant; only Fight is emitted).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature fights another target creature you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: fight_ability,
            })
            // Back face (Gold-Forge Garrison): "{4}, {T}: Create a 4/4 colorless Golem
            // artifact creature token." Gated to the back face (visible_face == 1).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, {T}: Create a 4/4 colorless Golem artifact creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: make_golem_token,
            })
            // Back face (Gold-Forge Garrison): "{T}: Add two mana of any one color."
            // One mana ability per WUBRG color, each adding two of that color.
            .with_activated_ability(two_mana_ability("{T}: Add {W}{W}.", add_white_mana))
            .with_activated_ability(two_mana_ability("{T}: Add {U}{U}.", add_blue_mana))
            .with_activated_ability(two_mana_ability("{T}: Add {B}{B}.", add_black_mana))
            .with_activated_ability(two_mana_ability("{T}: Add {R}{R}.", add_red_mana))
            .with_activated_ability(two_mana_ability("{T}: Add {G}{G}.", add_green_mana)),
    )
}

fn two_mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: Some(1), // back face only
        effect,
    }
}

fn add_white_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::White, ctx.source),
        ],
    }]
}
fn add_blue_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
        ],
    }]
}
fn add_black_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
        ],
    }]
}
fn add_red_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}
fn add_green_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}

/// Create a 4/4 colorless Golem artifact creature token.
fn make_golem_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").expect("Golem interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(golem);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: reg.interner().lookup("Golden Guardian").expect("name interned during register()"),
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn fight_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "When this creature dies this turn, return it to the battlefield transformed
    // under your control." — delayed conditional transform-return not expressible.
    vec![Effect::Fight {
        a: ctx.source,
        b: *id,
    }]
}
