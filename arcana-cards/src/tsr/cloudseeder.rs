//! Cloudseeder — `{1}{U}` 1/1 Faerie Spellshaper with Flying.
//!
//! Oracle:
//! * Flying.
//! * `{U}, {T}, Discard a card: Create a 1/1 blue Faerie creature token
//!   named Cloud Sprite. It has flying and "This token can block only
//!   creatures with flying."`
//!
//! The token is minted as a 1/1 blue Faerie with flying. Its
//! block-restriction static ("can block only creatures with flying") is
//! not expressible as a TokenDefinition ability, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Cloudseeder");
    let faerie = reg.interner_mut().intern("Faerie");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    // Pre-intern the token name + subtype so the resolver can recover them.
    let _cloud_sprite = reg.interner_mut().intern("Cloud Sprite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(spellshaper);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, {T}, Discard a card: Create a 1/1 blue Faerie creature token named Cloud Sprite. It has flying.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                tap: true,
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_cloud_sprite,
        }),
    )
}

fn make_cloud_sprite(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg
        .interner()
        .lookup("Cloud Sprite")
        .unwrap_or_default();
    let faerie = reg.interner().lookup("Faerie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    // GAP: token static "can block only creatures with flying" — block
    // restriction is not expressible on a TokenDefinition.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
