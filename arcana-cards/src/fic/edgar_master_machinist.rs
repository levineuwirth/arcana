//! Edgar, Master Machinist — `{2}{R}{W}` 2/4 Legendary Human Artificer
//! Noble.
//! "Once during each of your turns, you may cast an artifact spell from
//! your graveyard. If you cast a spell this way, that artifact enters
//! tapped." (a static cast-permission — GAP'd.)
//! Tools — Whenever Edgar attacks, it gets +X/+0 until end of turn, where
//! X is the greatest mana value among artifacts you control.
//!
//! No keyword line. Ability 1 is a static once-per-turn cast permission
//! with no expressible primitive (no permission/timing-window slot), so
//! it is GAP'd. Ability 2's trigger fires on attack, but X ("the greatest
//! mana value among artifacts you control") cannot be computed — there is
//! no greatest-mana-value script helper — so the pump amount is dynamic-
//! unexpressible and the effect is GAP'd (per the no-literal-for-dynamic
//! rule).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Edgar, Master Machinist");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP (static): "Once during each of your turns, you may cast an artifact
    // spell from your graveyard. If you cast a spell this way, that artifact
    // enters tapped." No once-per-turn cast-permission / timing-window slot.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: tools_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tools_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+0 where X is the greatest mana value among artifacts you
    // control" — no greatest-mana-value script helper, so X cannot be
    // computed; emitting a fixed literal would be a materially wrong card.
    Vec::new()
}
