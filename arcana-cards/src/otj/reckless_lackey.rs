//! Reckless Lackey — `{R}` 1/2 Goblin Pirate with First strike and Haste.
//!
//! Oracle:
//! * First strike, haste.
//! * `{2}{R}, Sacrifice this creature: Draw a card and create a Treasure
//!   token.`
//!
//! Keywords are base characteristics. The activated ability pays mana plus a
//! self-sacrifice and resolves to a draw and a Treasure (minted via the
//! commodity-token path so its activated ability is wired).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Lackey");
    let goblin = reg.interner_mut().intern("Goblin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, Sacrifice this creature: Draw a card and create a Treasure token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_treasure,
            }),
    )
}

fn draw_and_treasure(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::CreateCommodityToken {
            controller: ctx.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ]
}
