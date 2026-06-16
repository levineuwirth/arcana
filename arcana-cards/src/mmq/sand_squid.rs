//! Sand Squid — `{3}{U}` 2/2 Squid Beast.
//!
//! * Islandwalk — `KeywordAbility::Landwalk("Island")`.
//! * "You may choose not to untap this creature during your untap step."
//!   is a pure static replacement of the untap step — no trigger word, no
//!   activation cost, and no demonstrated primitive expresses an
//!   "optional don't-untap" static. GAP'd (see register).
//! * "{T}: Tap target creature. That creature doesn't untap during its
//!   controller's untap step for as long as this creature remains tapped."
//!   The tap is expressible (`Effect::Tap`); the lingering "doesn't untap
//!   while ~ stays tapped" rider has no demonstrated primitive, so only the
//!   tap is emitted.

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
    let name = reg.interner_mut().intern("Sand Squid");
    let squid = reg.interner_mut().intern("Squid");
    let beast = reg.interner_mut().intern("Beast");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squid);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(island)],
        ..Default::default()
    };

    // GAP: static — "You may choose not to untap this creature during your
    // untap step." No trigger / cost; no optional-skip-untap primitive.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tap target creature. That creature doesn't untap during its controller's untap step for as long as this creature remains tapped.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_target_creature,
            }),
    )
}

fn tap_target_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "doesn't untap ... as long as this creature remains tapped" rider
    // has no demonstrated primitive — only the tap is emitted.
    vec![Effect::Tap { target: *id }]
}
