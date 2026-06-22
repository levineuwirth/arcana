//! Whip Vine — `{2}{G}` 1/4 Plant Wall with Defender and Reach.
//! "You may choose not to untap this creature during your untap step."
//! "{T}: Tap target creature with flying blocked by this creature. That creature
//!  doesn't untap during its controller's untap step for as long as this creature
//!  remains tapped."
//!
//! GAP: "You may choose not to untap during your untap step" is a static untap
//! restriction with no expressible effect — omitted.
//! GAP: cannot restrict the target to "with flying blocked by this creature" (no
//! such target filter); targets any creature instead.
//! GAP: "doesn't untap ... for as long as this creature remains tapped" — the
//! conditional untap-lock rider is not expressible; only the one-shot Tap is
//! implemented.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Whip Vine");
    let plant = reg.interner_mut().intern("Plant");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "You may choose not to untap this creature during your untap step."

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Tap target creature with flying blocked by this creature. That \
                   creature doesn't untap during its controller's untap step for as long \
                   as this creature remains tapped."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_blocked_flyer,
        }),
    )
}

fn tap_blocked_flyer(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: untap-lock rider not expressible.
    vec![Effect::Tap { target: *id }]
}
