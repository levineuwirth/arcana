//! Sandals of Abdallah — `{4}` artifact (Arabian Nights, 1993).
//! "{2}, {T}: Target creature gains islandwalk until end of turn.
//! When that creature dies this turn, destroy this artifact."
//!
//! GAP: the delayed reflexive "when that creature dies this turn,
//! destroy this artifact" cannot be expressed — `DelayedAction`'s
//! `ThisDies` acts on the watched object itself, not a different
//! permanent — so only the islandwalk grant is wired.

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
    let name = reg.interner_mut().intern("Sandals of Abdallah");
    // Pre-intern the landwalk subtype for the resolver's lookup.
    let _island = reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Target creature gains islandwalk until end \
                       of turn. When that creature dies this turn, destroy \
                       this artifact."
                    .into(),
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
                effect: grant_islandwalk,
            },
        ),
    )
}

fn grant_islandwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(island) = reg.interner().lookup("Island") else {
        return Vec::new();
    };
    // GAP: 'When that creature dies this turn, destroy this artifact' —
    // no delayed trigger that watches one object and acts on another.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Landwalk(island),
        duration: Duration::EndOfTurn,
    }]
}
