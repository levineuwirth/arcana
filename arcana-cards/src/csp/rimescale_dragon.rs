//! Rimescale Dragon — `{5}{R}{R}` 5/5 Snow Creature — Dragon with Flying.
//! "{2}{S}: Tap target creature and put an ice counter on it."
//! "Creatures with ice counters on them don't untap during their controllers'
//!  untap steps." (static — GAP)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rimescale Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let _ice = reg.interner_mut().intern("ice");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP static: "Creatures with ice counters on them don't untap during their
    // controllers' untap steps." — untap restriction, no API surface.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{S}: Tap target creature and put an ice counter on it.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{S}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_and_ice,
        }),
    )
}

fn tap_and_ice(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let Some(ice) = reg.interner().lookup("ice").map(CounterKind::Named) else {
        return vec![Effect::Tap { target: *id }];
    };
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: ice,
            count: 1,
        },
    ]
}
