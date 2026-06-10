//! Thought Dissector — `{4}` artifact.
//! "{X}, {T}: Target opponent reveals cards from the top of their
//! library until an artifact card or X cards are revealed, whichever
//! comes first. If an artifact card is revealed this way, put it onto
//! the battlefield under your control and sacrifice this artifact. Put
//! the rest of the revealed cards into that player's graveyard."
//! Modeled with `Effect::RevealUntil` over the targeted player's
//! library (max_reveal = X, found artifact to the battlefield, rest to
//! the graveyard). Fidelity gaps are documented in the resolver.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thought Dissector");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}, {T}: Target opponent reveals cards from the top of their library until an artifact card or X cards are revealed, whichever comes first. If an artifact card is revealed this way, put it onto the battlefield under your control and sacrifice this artifact. Put the rest of the revealed cards into that player's graveyard.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reveal_for_artifact,
        }),
    )
}

fn reveal_for_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = ctx.x_value.unwrap_or(0);
    // GAP: 'target opponent' — opponent-only constraint on a player
    // target is not expressible; any player may be chosen.
    // GAP: the found artifact is put onto the battlefield under the
    // revealing player's control (no "under your control" rider on
    // RevealUntil) and the conditional "sacrifice this artifact" rider
    // is not expressible.
    vec![Effect::RevealUntil {
        player: *p,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::Graveyard,
        max_reveal: Some(x),
    }]
}
