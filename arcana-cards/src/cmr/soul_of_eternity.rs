//! Soul of Eternity — `{5}{W}{W}` */* Avatar with Encore {7}{W}{W}.
//!
//! Soul of Eternity's power and toughness are each equal to your life total.
//! Encore {7}{W}{W} ({7}{W}{W}, Exile this card from your graveyard: For each
//! opponent, create a token copy that attacks that opponent this turn if able.
//! They gain haste. Sacrifice them at the beginning of the next end step.
//! Activate only as a sorcery.)
//!
//! The */* CDA ("P/T equal to your life total") is recorded with
//! `PtValue::Star`; the characteristic-defining static that sets the star
//! value to your life total has no expressible declarative form and is GAP'd.
//! Encore is not a `KeywordAbility` variant; it is modeled as a
//! graveyard-activated ability with cost {7}{W}{W} + exile-self, but its
//! effect (per-opponent attacking token copies, haste, end-step sacrifice,
//! sorcery-speed) cannot be assembled from the available primitives and is
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Eternity");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    // GAP: CDA static — "power and toughness are each equal to your life
    //      total"; recorded as */* via PtValue::Star.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Encore {7}{W}{W} ({7}{W}{W}, Exile this card from your graveyard: For each opponent, create a token copy that attacks that opponent this turn if able. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.)".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{W}{W}").expect("valid cost"),
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
    // gain haste, and are sacrificed at the next end step is not expressible
    // (no "token copy of a graveyard card that attacks a specific opponent"
    // primitive).
    Vec::new()
}
