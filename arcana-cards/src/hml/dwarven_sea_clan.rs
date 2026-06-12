//! Dwarven Sea Clan — `{2}{R}` 1/1 red Dwarf.
//! "{T}: Choose target attacking or blocking creature whose controller controls an Island.
//! This creature deals 2 damage to that creature at end of combat. Activate only before
//! the end of combat step."
//! GAP: "attacking or blocking" + "controller controls an Island" TargetFilter constraint
//! not expressible; "activate only before the end of combat step" timing window not enforced.
//! GAP (narrow): "at end of combat" timing approximated as the next end STEP
//! (DelayedWhen has no end-of-combat slot); same turn, slightly later.

use arcana_core::effects::{DelayedWhen, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::PendingTrigger;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Sea Clan");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose target attacking or blocking creature whose controller controls an Island. This creature deals 2 damage to that creature at end of combat.".into(),
                cost: ActivationCost::tap_only(),
                // GAP: "attacking or blocking creature whose controller controls an Island"
                // not expressible; using target_creature() as approximation.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: delayed_damage_at_eoc,
            }),
    )
}

fn delayed_damage_at_eoc(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP (narrow): "at end of combat" approximated as the next end
    // step (DelayedWhen has no end-of-combat slot). The chosen creature
    // rides the `source` slot; the delayed fn no-ops if it left play.
    vec![Effect::ScheduleDelayedEffect {
        source: *id,
        controller: ctx.controller,
        when: DelayedWhen::NextEndStep,
        effect: delayed_two_damage,
    }]
}

/// "This creature deals 2 damage to that creature at end of combat."
/// `pt.source` is the chosen creature.
/// GAP (narrow): the Sea Clan's own id isn't carried by the scheduler
/// (the chosen creature occupies the `source` slot), so damage
/// attribution falls back to the damaged creature itself.
fn delayed_two_damage(
    state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    if state.objects.get(pt.source).is_none() {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: pt.source,
        target: DamageTarget::Object(pt.source),
        amount: 2,
    }]
}
