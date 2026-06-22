//! Rotwidow Pack — `{2}{B}{G}` 2/4 Spider with Reach.
//! Reach.
//! {3}{B}{G}, Exile a creature card from your graveyard: Create a 1/2 green
//! Spider creature token with reach, then each opponent loses 1 life for each
//! Spider you control.
//!
//! Reach is wired. The activated ability's mana cost is wired and the payload
//! (create the Spider token, then each opponent loses 1 life per Spider you
//! control) is fully wired. GAP: the additional "Exile a creature card from your
//! graveyard" cost has no ActivationCost field (no graveyard-exile-other cost).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rotwidow Pack");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{B}{G}, Exile a creature card from your graveyard: Create a 1/2 green Spider creature token with reach, then each opponent loses 1 life for each Spider you control.".into(),
            // GAP: additional cost "Exile a creature card from your graveyard"
            // has no ActivationCost field — only the mana portion is modeled.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{B}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_spider_and_drain,
        }),
    )
}

fn make_spider_and_drain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spider = reg.interner().lookup("Spider").unwrap_or_default();
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spider);

    let mut effects = vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: spider,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Reach],
            abilities: vec![],
        },
    }];

    // each opponent loses 1 life for each Spider you control
    let spider_filter =
        script::subtype_filter(reg, "Spider").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &spider_filter, ctx.controller);
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::LoseLife {
            player: opp,
            amount: n,
        });
    }
    effects
}
