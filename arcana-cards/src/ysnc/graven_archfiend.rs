//! Graven Archfiend — `{3}{B}{B}` 4/5 Artifact Creature — Gargoyle Demon
//! with Flying.
//!
//! Oracle text:
//! * "As an additional cost to cast this spell, you may sacrifice a non-Demon
//!   creature." — an optional additional CAST cost; not expressible as a cast
//!   cost in this card class, GAP'd below.
//! * Flying — base keyword.
//! * "When Graven Archfiend enters the battlefield, if its additional cost was
//!   paid, conjure a card named Demonic Pact onto the battlefield." — an ETB
//!   trigger; Conjure is an Arena-only mechanic with no Effect variant, so the
//!   effect is GAP'd while the trigger is still emitted.

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
    let name = reg.interner_mut().intern("Graven Archfiend");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As an additional cost to cast this spell, you may sacrifice a
    // non-Demon creature." — optional additional cast cost not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_conjure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need
    // registry-by-name lookup in Effect::execute). The "if additional cost
    // was paid" gate is also not tracked.
    Vec::new()
}
