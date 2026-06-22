//! Jan Jansen, Chaos Crafter — `{R}{W}{B}` 3/3 Legendary Gnome
//! Artificer with Haste.
//!
//! Oracle:
//! * Haste — base keyword. (Scryfall also tags "Treasure", which is a
//!   token type, not a `KeywordAbility`.)
//! * `{T}, Sacrifice an artifact creature: Create two Treasure
//!   tokens.`
//! * `{T}, Sacrifice a noncreature artifact: Create two 1/1 colorless
//!   Construct artifact creature tokens.`

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jan Jansen, Chaos Crafter");
    let gnome = reg.interner_mut().intern("Gnome");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the Construct subtype for the token resolver.
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: arcana_core::types::SupertypeSet(
            arcana_core::types::SupertypeSet::LEGENDARY,
        ),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact creature: Create two Treasure tokens.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_treasures,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a noncreature artifact: Create two 1/1 colorless Construct artifact creature tokens.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_constructs,
            }),
    )
}

fn make_treasures(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Treasure,
        count: 2,
    }]
}

fn make_constructs(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let construct = reg.interner().lookup("Construct").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: construct,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
    ]
}
