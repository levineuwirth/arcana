//! Voracious Fell Beast — `{4}{B}{B}` 4/4 Drake Beast with Flying.
//!
//! "When this creature enters, each opponent sacrifices a creature of
//! their choice. Create a Food token for each creature sacrificed this
//! way."
//!
//! The per-opponent sacrifice is expressible; the "create a Food token
//! for each creature sacrificed this way" rider is GAP'd — there is no
//! way to read back how many permanents an `Effect::Sacrifice` actually
//! sacrificed to feed a dynamic token count.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voracious Fell Beast");
    let drake = reg.interner_mut().intern("Drake");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_each_opponent_sacrifices,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_each_opponent_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Create a Food token for each creature sacrificed this way" —
    // no read-back of the actual sacrifice count to drive a dynamic
    // token mint.
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }
    effects
}
