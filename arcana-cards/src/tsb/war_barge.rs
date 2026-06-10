//! War Barge — `{4}` artifact.
//! "{3}: Target creature gains islandwalk until end of turn. When this
//! artifact leaves the battlefield this turn, destroy that creature.
//! A creature destroyed this way can't be regenerated."
//! The islandwalk grant is wired via `KeywordAbility::Landwalk`.
//! GAP: the delayed "when this artifact leaves the battlefield this
//! turn, destroy that creature" rider (and its no-regeneration clause)
//! is not expressible — `DelayedAction` cannot watch the SOURCE
//! leaving while acting on the TARGET.

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
    let name = reg.interner_mut().intern("War Barge");
    let _island = reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: Target creature gains islandwalk until end of turn. When this artifact leaves the battlefield this turn, destroy that creature. A creature destroyed this way can't be regenerated.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_islandwalk,
        }),
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
    // GAP: "When this artifact leaves the battlefield this turn,
    // destroy that creature. A creature destroyed this way can't be
    // regenerated." — no delayed trigger watches the source leaving
    // while acting on the target.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Landwalk(island),
        duration: Duration::EndOfTurn,
    }]
}
