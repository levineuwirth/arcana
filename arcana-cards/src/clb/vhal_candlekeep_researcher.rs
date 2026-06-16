//! Vhal, Candlekeep Researcher — `{3}{U}` Legendary 2/3 Human Wizard
//! with Vigilance.
//!
//! Oracle text:
//! * Vigilance.
//! * "{T}: Add an amount of {C} equal to Vhal's toughness. This mana
//!   can't be spent to cast spells from your hand." — a tap mana
//!   ability producing colorless mana equal to this creature's
//!   toughness. GAP: the "can't be spent to cast spells from your
//!   hand" mana-restriction rider is not expressible.
//! * "Choose a Background" — GAP: not an expressible ability (the
//!   Background partner-commander deck-building keyword is not modeled).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vhal, Candlekeep Researcher");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Choose a Background" — Background partner-commander keyword
    // not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "This mana can't be spent to cast spells from your
                // hand" mana-restriction rider is not expressible.
                text: "{T}: Add an amount of {C} equal to Vhal's toughness."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_equal_to_toughness,
            }),
    )
}

fn add_colorless_equal_to_toughness(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::toughness_of(state, ctx.source).max(0) as usize;
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); n],
    }]
}
