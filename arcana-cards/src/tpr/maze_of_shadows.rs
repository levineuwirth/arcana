//! Maze of Shadows — nonbasic land (Tempest, 1997).
//! "{T}: Add {C}." and "{T}: Untap target attacking creature with
//! shadow. Prevent all combat damage that would be dealt to and dealt
//! by that creature this turn." The untap plus damage-to-it
//! prevention is wired with approximations; see the GAP comments.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze of Shadows");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Untap target attacking creature with shadow. \
                       Prevent all combat damage that would be dealt to \
                       and dealt by that creature this turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: "attacking creature WITH SHADOW" — neither an
                // attacking-status nor a has-keyword refinement exists
                // in this catalog; plain target creature declared.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_and_fog,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn untap_and_fog(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Approximation: PreventDamage with amount None prevents ALL
    // damage TO the creature (oracle says combat damage only — no
    // combat-only flag exists on PreventDamage).
    // GAP: "and dealt BY that creature" — prevention of damage dealt
    // by one specific object is not expressible (PreventDamageFrom is
    // filter-based, not id-based).
    vec![
        Effect::Untap { target: *id },
        Effect::PreventDamage {
            target: DamageTarget::Object(*id),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
