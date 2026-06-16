//! Reptilian Recruiter — `{3}{R}{R}` 4/2 Creature — Lizard Warrior.
//! Trample.
//! When this creature enters, choose target creature. If that
//!   creature's power is 2 or less or if you control another Lizard,
//!   gain control of that creature until end of turn, untap it, and it
//!   gains haste until end of turn.
//!
//! Decomposition:
//! * keyword line → Trample.
//! * ETB trigger targeting a creature. The "if power 2 or less OR you
//!   control another Lizard" is a resolution-time condition on the
//!   chosen target, computed in the effect fn (script::power_of +
//!   script::count_matching). When satisfied we emit the Threaten
//!   suite: ChangeControlEot + Untap + grant Haste EOT.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reptilian Recruiter");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_steal_conditional,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn etb_steal_conditional(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let power_ok = script::power_of(state, *id) <= 2;
    let lizard_filter = script::subtype_filter(reg, "Lizard")
        .controlled_by(ControllerConstraint::You);
    // "another Lizard" — exclude this creature from the count.
    let lizards = script::ids_matching(state, &lizard_filter, trig.controller)
        .into_iter()
        .filter(|lid| *lid != trig.source)
        .count();
    if !(power_ok || lizards >= 1) {
        return Vec::new();
    }
    vec![
        Effect::ChangeControlEot { target: *id, new_controller: trig.controller },
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
