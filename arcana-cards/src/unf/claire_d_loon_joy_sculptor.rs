//! Claire D'Loon, Joy Sculptor — `{W}{U}` 2/3 Legendary Human Performer.
//!
//! Oracle:
//! * When Claire D'Loon enters, for the rest of the game, tokens you own
//!   become cards that are still tokens. (GAP — "tokens become cards"
//!   is a game-wide replacement static with no expressible Effect.)
//! * {1}{W}{U}, {T}: You may put a token from your graveyard or hand
//!   onto the battlefield. (Partial — only the hand half is expressible
//!   via PutFromHandOntoBattlefield; the graveyard half is GAP'd.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Claire D'Loon, Joy Sculptor");
    let human = reg.interner_mut().intern("Human");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tokens_become_cards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}{U}, {T}: You may put a token from your graveyard or hand onto the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_token_from_hand,
            }),
    )
}

fn etb_tokens_become_cards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for the rest of the game, tokens you own become cards that
    // are still tokens" — a game-wide rule-altering replacement static
    // with no expressible Effect variant.
    Vec::new()
}

fn put_token_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "from your graveyard" half is not expressible; only the
    // hand half is modeled via PutFromHandOntoBattlefield.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::default().tokens_only(),
        tapped: false,
    }]
}
