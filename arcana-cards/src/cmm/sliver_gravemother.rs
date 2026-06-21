//! Sliver Gravemother — `{W}{U}{B}{R}{G}` 6/6 Legendary Sliver with Encore.
//!
//! The "legend rule" doesn't apply to Slivers you control.
//! Each Sliver creature card in your graveyard has encore {X}, where X is its
//! mana value.
//! Encore {5} ({5}, Exile this card from your graveyard: For each opponent,
//! create a token copy that attacks that opponent this turn if able. They gain
//! haste. Sacrifice them at the beginning of the next end step. Activate only
//! as a sorcery.)
//!
//! Both statics (the legend-rule exemption and the graveyard-wide encore
//! grant) are pure continuous abilities with no expressible form and are
//! GAP'd. Encore is not a `KeywordAbility` variant; it is modeled as a
//! graveyard-activated ability with cost {5} + exile-self, but its
//! per-opponent attacking-token-copy body cannot be assembled from the
//! available primitives and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sliver Gravemother");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    // GAP: static — "the legend rule doesn't apply to Slivers you control".
    // GAP: static — "Each Sliver creature card in your graveyard has encore
    //      {X}, where X is its mana value".
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Encore {5} ({5}, Exile this card from your graveyard: For each opponent, create a token copy that attacks that opponent this turn if able. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.)".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: encore_effect,
        }),
    )
}

fn encore_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Encore body — per-opponent token copies that attack that opponent,
    // gain haste, and are sacrificed at the next end step is not expressible.
    Vec::new()
}
