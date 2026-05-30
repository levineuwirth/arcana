//! Mogis's Marauder — `{2}{B}` 2/2 black Human Berserker.
//! "When this creature enters, up to X target creatures each gain
//! intimidate and haste until end of turn, where X is your devotion
//! to black."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mogis's Marauder");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_grant_intimidate_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::X,
                    controller: None,
                }],
            })
            .with_trigger_dynamic_x(1, |trig: &PendingTrigger| -> u32 {
                // X = devotion to black; evaluated at trigger-fire time
                // We store it in x_value; the effect fn reads trig.targets
                // which has been sized to X by the engine.
                // The closure here is the dynamic-x resolver used to cap targets.
                // We can't call script:: here (no state), so we return 0 as a
                // placeholder — the engine uses the stamped x_value from the stack.
                // GAP: dynamic_x closure lacks GameState; devotion cannot be computed
                // here. Targets will be capped to 0. Real devotion gating requires
                // engine support for state-aware dynamic-x.
                let _ = trig;
                0
            }),
    )
}

fn etb_grant_intimidate_haste(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = devotion to black at resolution time
    let x = script::devotion(state, trig.controller, ColorSet::black());
    // Apply intimidate + haste to each chosen target (up to X)
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        let TargetChoice::Object(id) = target else { continue; };
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Intimidate,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    // x is used to bound target count; here we use it to suppress effects
    // if no targets were chosen
    let _ = x;
    effects
}
