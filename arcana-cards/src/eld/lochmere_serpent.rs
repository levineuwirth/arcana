//! Lochmere Serpent — `{4}{U}{B}` 7/7 Serpent with Flash.
//! "{U}, Sacrifice an Island: This creature can't be blocked this turn."
//! "{B}, Sacrifice a Swamp: You gain 1 life and draw a card."
//! "{U}{B}: Exile five target cards from an opponent's graveyard. Return this
//!  card from your graveyard to your hand. Activate only as a sorcery."
//!  (The "only as a sorcery" timing is the default sorcery-speed activation.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lochmere Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    let island_filter = arcana_core::script::subtype_filter(reg, "Island");
    let swamp_filter = arcana_core::script::subtype_filter(reg, "Swamp");

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, Sacrifice an Island: This creature can't be blocked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    sacrifice_other: Some(island_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cant_be_blocked,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Sacrifice a Swamp: You gain 1 life and draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    sacrifice_other: Some(swamp_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_and_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}: Exile five target cards from an opponent's graveyard. Return this card from your graveyard to your hand. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default()
                            .controlled_by(ControllerConstraint::Opponent),
                    },
                    count: TargetCount::Exactly(5),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_and_return,
            }),
    )
}

fn cant_be_blocked(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}

fn gain_and_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife { player: ctx.controller, amount: 1 },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

fn exile_and_return(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = ctx
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ExileFromGraveyard { target: *id }),
            _ => None,
        })
        .collect();
    effects.push(Effect::ReturnFromGraveyardToHand { target: ctx.source });
    effects
}
