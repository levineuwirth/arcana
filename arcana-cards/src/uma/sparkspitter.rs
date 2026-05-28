//! Sparkspitter — `{2}{R}` 1/3 Elemental Spellshaper.
//! `{R}, {T}, Discard a card: Create a 3/1 red Elemental with trample, haste, and "At the
//! beginning of the end step, sacrifice this token."`
//! GAP: "Discard a card" activation cost — no discard-any-card field in ActivationCost.
//! GAP: token's "sacrifice at end step" triggered ability not wired in TokenDefinition.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sparkspitter");
    let elemental = reg.interner_mut().intern("Elemental");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, {T}, Discard a card: Create a 3/1 red Elemental with trample, haste.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    tap: true,
                    // GAP: "discard a card" cost not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_spark_elemental,
            }),
    )
}

fn create_spark_elemental(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elemental);
    // GAP: token's "sacrifice at beginning of end step" ability not wired
    vec![Effect::CreateTokenSacEot {
        controller: ctx.controller,
        token: TokenDefinition {
            name: elemental,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}
