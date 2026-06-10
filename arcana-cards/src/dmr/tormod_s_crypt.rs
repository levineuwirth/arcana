//! Tormod's Crypt — `{0}` artifact (The Dark, 1994).
//! "{T}, Sacrifice this artifact: Exile target player's graveyard."
//! The mandatory exile-ALL is approximated with the engine's
//! any-number zone pick (see fidelity note in the resolver) — the
//! catalog has no exile-entire-zone effect.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tormod's Crypt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice this artifact: Exile target player's \
                       graveyard."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_graveyard,
            },
        ),
    )
}

fn exile_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Fidelity note: "exile target player's graveyard" is mandatory and
    // total; the catalog has no exile-all-zone effect, so this is modeled
    // as the controller's min-0/max-all pick over that graveyard
    // (ChooseAnyNumberFromZone + PickAction::Exile).
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: ctx.controller,
        zone: Zone::Graveyard(*p),
        filter: ObjectFilter::default(),
        action: PickAction::Exile,
    }]
}
