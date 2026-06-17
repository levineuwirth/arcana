//! Lluwen, Imperfect Naturalist — `{B/G}{B/G}` 1/3 Legendary Creature — Elf Druid.
//! When Lluwen enters, mill four cards, then you may put a creature or land card
//!   from among the milled cards on top of your library. (Mill is expressible;
//!   the "put one milled card on top" selection is GAP'd.)
//! {2}{B/G}{B/G}{B/G}, {T}, Discard a land card: Create a 1/1 black and green
//!   Worm creature token for each land card in your graveyard.
//!
//! Mill is not an expressible KeywordAbility variant (keywords: vec![]).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lluwen, Imperfect Naturalist");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let _worm = reg.interner_mut().intern("Worm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // discard-cost filter: a land card from hand.
    let discard_land = ObjectFilter {
        types: Some(TypeLine::LAND.into()),
        ..ObjectFilter::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B/G}{B/G}{B/G}, {T}, Discard a land card: Create a 1/1 black and green Worm creature token for each land card in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B/G}{B/G}{B/G}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(discard_land),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_worms,
            }),
    )
}

fn etb_mill(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may put a creature or land card from among the milled cards on
    // top of your library" — no expressible selection effect for milled cards.
    vec![Effect::Mill {
        player: trig.controller,
        count: 4,
    }]
}

fn make_worms(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // N = number of land cards in your graveyard.
    let land_filter = ObjectFilter {
        types: Some(TypeLine::LAND.into()),
        ..ObjectFilter::default()
    };
    let n = script::graveyard_matching(state, &land_filter, ctx.controller, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    let worm = reg.interner().lookup("Worm").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worm);
    let token = TokenDefinition {
        name: worm,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        })
        .collect()
}
