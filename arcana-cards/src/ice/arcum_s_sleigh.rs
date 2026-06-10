//! Arcum's Sleigh — `{1}` artifact (Ice Age).
//! "{2}, {T}: Target creature gains vigilance until end of turn.
//! Activate only during combat and only if defending player controls
//! a snow land."
//!
//! GAP: the activation restrictions ("only during combat and only if
//! defending player controls a snow land") are not expressible —
//! `ActivationCost` has no timing window or condition field; the
//! ability is activatable at normal sorcery speed unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcum's Sleigh");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Target creature gains vigilance until end \
                       of turn. Activate only during combat and only if \
                       defending player controls a snow land."
                    .into(),
                // GAP: "Activate only during combat and only if defending
                // player controls a snow land" — no timing-window or
                // activation-condition field on ActivationCost.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_vigilance,
            },
        ),
    )
}

fn grant_vigilance(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Vigilance,
        duration: Duration::EndOfTurn,
    }]
}
