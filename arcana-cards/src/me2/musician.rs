//! Musician — `{2}{U}` 1/3 blue Human Wizard.
//! Cumulative upkeep {1}.
//! {T}: Put a music counter on target creature. If it doesn't have "At the
//! beginning of your upkeep, destroy this creature unless you pay {1} for each
//! music counter on it," it gains that ability.
//!
//! Cumulative upkeep is not a base KeywordAbility variant (keywords: vec![]) and
//! has no expressible form — GAP'd. The tap activation puts a music counter
//! (CounterKind::Named) on a targeted creature; the conditional "it gains that
//! ability" grant is GAP'd (the granted ability has a per-music-counter dynamic
//! pay-or-destroy cost that OptionalPayment can't express).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Musician");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _music = reg.interner_mut().intern("music");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Cumulative upkeep {1}" — not a base KeywordAbility and no
    //      expressible upkeep-tax / age-counter machinery.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Put a music counter on target creature. If it doesn't \
                   have \"At the beginning of your upkeep, destroy this creature \
                   unless you pay {1} for each music counter on it,\" it gains \
                   that ability."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_music_counter,
        }),
    )
}

fn put_music_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it doesn't have [the upkeep destroy-unless-pay {1} per music
    //      counter] ability, it gains that ability" — the granted ability's
    //      per-counter dynamic pay-or-destroy cost is not expressible.
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let Some(music) = reg.interner().lookup("music") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(music),
        count: 1,
    }]
}
