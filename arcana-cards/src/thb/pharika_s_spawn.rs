//! Pharika's Spawn — `{3}{B}` 3/4 Gorgon.
//! "Escape—{5}{B}, Exile three other cards from your graveyard.
//!  This creature escapes with two +1/+1 counters on it. When it enters this
//!  way, each opponent sacrifices a non-Gorgon creature of their choice."
//!
//! GAP (keyword/alt-cost): Escape is not a usable KeywordAbility variant and
//! the graveyard alternative cost is not expressible — `keywords: vec![]`.
//! GAP (counters): "escapes with two +1/+1 counters" is part of escape
//! resolution (only when cast from graveyard) — not expressible.
//! Partial: the ETB sacrifice is modeled on a plain SelfEntersBattlefield
//! trigger; the "only when it escaped (entered this way)" gate is dropped, so
//! the opponent-sacrifice fires on any ETB rather than only escape ETBs.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Pharika's Spawn");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::creature();
    if let Some(gorgon) = reg.interner().lookup("Gorgon") {
        filter = filter.without_subtype_sym(gorgon);
    }
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::Sacrifice {
            player: opp,
            filter: filter.clone(),
            count: 1,
        })
        .collect()
}
