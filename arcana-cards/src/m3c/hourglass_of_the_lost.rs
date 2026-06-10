//! Hourglass of the Lost — `{2}{W}` artifact (The Lost Caverns of
//! Ixalan). "{T}: Add {W}. Put a time counter on this artifact." and
//! "{T}, Remove X time counters from this artifact and exile it:
//! Return each nonland permanent card with mana value X from your
//! graveyard to the battlefield. Activate only as a sorcery." The
//! first ability is wired (marked non-mana because its effect is not
//! solely AddMana); the second ability's variable remove-X-counters +
//! exile-self cost is not expressible and the whole line is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hourglass of the Lost");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    // GAP: "{T}, Remove X time counters from this artifact and exile it:
    // Return each nonland permanent card with mana value X from your graveyard
    // to the battlefield. Activate only as a sorcery." — a variable
    // remove-X-counters cost is not expressible (remove_self_counter is a
    // fixed (kind, n)), so the whole second activated ability is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {W}. Put a time counter on this artifact.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white_and_time_counter,
            },
        ),
    )
}

fn add_white_and_time_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Time,
            count: 1,
        },
    ]
}
