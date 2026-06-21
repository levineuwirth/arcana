//! Pharika, God of Affliction — `{1}{B}{G}` 5/5 Legendary Enchantment
//! Creature — God.
//! Indestructible.
//! As long as your devotion to black and green is less than seven,
//! Pharika isn't a creature. — GAP (devotion-gated type-removal static).
//! {B}{G}: Exile target creature card from a graveyard. Its owner
//! creates a 1/1 black and green Snake enchantment creature token with
//! deathtouch.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pharika, God of Affliction");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    // Snake token primary subtype (interned for the resolver lookup).
    let _snake = reg.interner_mut().intern("Snake");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        // GAP: "As long as your devotion to black and green is less than
        // seven, Pharika isn't a creature" — devotion-gated type-removal
        // static not expressible here.
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}{G}: Exile target creature card from a graveyard. Its owner creates a 1/1 black and green Snake enchantment creature token with deathtouch.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exile_and_make_snake,
        }),
    )
}

fn exile_and_make_snake(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let owner = script::target_controller(state, *id, ctx.controller);
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(snake);
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CreateToken {
            controller: owner,
            token: arcana_core::effects::TokenDefinition {
                name: snake,
                colors: ColorSet::black() | ColorSet::green(),
                types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Deathtouch],
                abilities: vec![],
            },
        },
    ]
}
