//! Scarwood Hag — `{1}{G}` 1/1 Hag.
//! {G}{G}{G}{G}, {T}: Target creature gains forestwalk until end of turn.
//! {T}: Target creature loses forestwalk until end of turn.
//!
//! The grant-forestwalk ability is wired (Landwalk(Forest) until end of
//! turn). The lose-forestwalk ability is GAP'd: there is no effect to
//! remove a single named keyword (LoseAllAbilities would strip every
//! ability, which is wrong).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scarwood Hag");
    let hag = reg.interner_mut().intern("Hag");
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{G}{G}{G}, {T}: Target creature gains forestwalk until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}{G}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_forestwalk,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature loses forestwalk until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lose_forestwalk_gap,
            }),
    )
}

fn grant_forestwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(forest) = reg.interner().lookup("Forest") else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Landwalk(forest),
        duration: Duration::EndOfTurn,
    }]
}

fn lose_forestwalk_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no effect removes a single named keyword (forestwalk) from a
    // creature; LoseAllAbilities would strip everything.
    Vec::new()
}
