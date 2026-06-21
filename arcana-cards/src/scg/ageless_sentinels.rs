//! Ageless Sentinels — `{3}{W}` 4/4 Wall with Flying and Defender.
//!
//! Oracle:
//! * Defender
//! * Flying
//! * When this creature blocks, it becomes a Bird Giant, and it loses defender.
//!   (It's no longer a Wall. This effect lasts indefinitely.)
//!
//! Flying and Defender are base keywords. The blocks trigger's effect —
//! becoming a Bird Giant (subtype replacement) and losing Defender — has no
//! expressible primitives: there is no add/replace-subtype effect, and no
//! targeted lose-a-single-keyword effect (LoseAllAbilities would also strip
//! Flying). The trigger condition is recorded, but its effect is GAP'd rather
//! than emitting a wrong partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ageless Sentinels");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: become_bird_giant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn become_bird_giant(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it becomes a Bird Giant, and it loses defender." No add/replace
    // subtype effect and no targeted single-keyword removal (LoseAllAbilities
    // would also strip Flying), so the effect is GAP'd.
    Vec::new()
}
