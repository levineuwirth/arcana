//! Grabby Giant // That's Mine — `{3}{R}` // `{1}{R}` red Adventure creature.
//! Creature: 4/3 Giant. Reach. {2}{R}, Sacrifice an artifact or land: Draw a card.
//! Adventure (That's Mine — Instant): Create a Treasure token.
//! GAP: "{2}{R}, Sacrifice an artifact or land: Draw a card" — activated ability with sacrifice-specific-type cost not in ActivationCost (sacrifice field is generic permanent only).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grabby Giant");
    let adv_name = reg.interner_mut().intern("That's Mine");
    let giant_sub = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a Treasure token.".into(),
        target_requirements: vec![],
        modal: None,
        effect: thats_mine_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, Sacrifice an artifact or land: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").unwrap(),
                    sacrifice: true, // GAP: sacrifice-artifact-or-land not modeled specifically
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: sac_draw,
            })
            .with_adventure(adventure),
    )
}

fn sac_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn thats_mine_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: entry.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
