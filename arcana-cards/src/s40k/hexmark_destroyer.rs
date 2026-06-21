//! Hexmark Destroyer — `{4}{B}{B}` 6/6 Artifact Creature — Necron.
//! Multi-threat Eliminator — This creature can't be blocked except by six or
//! more creatures. (GAP: no count-gated block restriction effect.)
//! Unearth {4}{B}{B} — modeled as a graveyard-activated ability returning this
//! card to the battlefield with haste, then exiling it at the next end step.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::effects::{DelayedAction, DelayedWhen};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hexmark Destroyer");
    let necron = reg.interner_mut().intern("Necron");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: keyword — neither "Multi-threat Eliminator" nor "Unearth" is an
        // available KeywordAbility variant.
        ..Default::default()
    };

    // GAP: static — "can't be blocked except by six or more creatures" has no
    // count-gated block-restriction Effect (only the absolute CantBeBlocked).

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Unearth {4}{B}{B} ({4}{B}{B}: Return this card from your \
                   graveyard to the battlefield. It gains haste. Exile it at the \
                   beginning of the next end step or if it would leave the \
                   battlefield. Unearth only as a sorcery.)"
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{B}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: unearth,
        }),
    )
}

fn unearth(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::ReturnFromGraveyardToBattlefield { target: ctx.source },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ])]
}
