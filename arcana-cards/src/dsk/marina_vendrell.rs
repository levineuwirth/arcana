//! Marina Vendrell — `{W}{U}{B}{R}{G}` 3/5 Legendary Human Warlock.
//!
//! Oracle:
//! * When Marina Vendrell enters, reveal the top seven cards of your library.
//!   Put all enchantment cards from among them into your hand and the rest on
//!   the bottom of your library in a random order.
//! * {T}: Lock or unlock a door of target Room you control. Activate only as a
//!   sorcery.
//!
//! Both effects are GAP'd:
//! * The ETB reveals seven and takes ALL enchantments — `DigTopN` is
//!   single-take and `RevealUntil` takes only the first match, so a
//!   "put all matching" reveal-top-N is not expressible. The trigger is kept
//!   wired with a GAP body.
//! * The {T} ability manipulates Room doors (the Room/door mechanic) — no
//!   expressible primitive.

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
    let name = reg.interner_mut().intern("Marina Vendrell");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "{T}: Lock or unlock a door of target Room you control." — the
    // Room/door mechanic has no expressible primitive.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_reveal_seven,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_reveal_seven(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal top seven, put ALL enchantment cards into hand, rest on
    // bottom in random order" — no multi-take filtered reveal-top-N primitive
    // (DigTopN is single-take; RevealUntil takes only the first match).
    Vec::new()
}
